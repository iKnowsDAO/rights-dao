import {ElMessage} from 'element-plus/es';
import {t} from '@/locale';

// 指定一些缓存时间
export class TTL {
    static minute1 = 60;
    static minute10 = 60 * 10;
    static minute15 = 60 * 15;
    static minute30 = 60 * 30;
    static hour1 = 60 * 60;
    static hour2 = 60 * 60 * 2;
    static hour4 = 60 * 60 * 4;
    static hour6 = 60 * 60 * 6;
    static hour12 = 60 * 60 * 12;
    static day1 = 60 * 60 * 24;
    static day3 = 60 * 60 * 24 * 3;
    static day7 = 60 * 60 * 24 * 7;

    static minute = (minute: number) => 60 * minute;
    static hour = (hour: number) => 60 * 60 * hour;
    static day = (day: number) => 60 * 60 * 24 * day;
}

// 直接获取本地储存的带过期时间的数据进行验证，如果没有才发出网络请求
// key: 设置storage里的key，注意：key 里面应当包含执行方法的参数信息，不同的参数不能共用一个 key
// execute: 传入执行方法
// ttl: Time To Live 过期时长，秒为单位，比如xx秒以后过期
// isLocal: true为LocalStorage，false为非LocalStorage
// 方法返回值必须是res.Ok才会激活本地缓存
export async function getCache(info: {
    key: string;
    execute: () => Promise<any>;
    ttl?: number;
    isLocal?: boolean;
    timeout?: number; // 超时限制，如果网络请求时间实在太长，就提示错误吧
    cache?: boolean; // 是否读取旧缓存，而是加载新的数据，再将新的数据缓存，如果设置 cache 是 false，那么表明不使用缓存
    notice?: (_fromCaching: boolean) => void; // 万一上级需要判断是否从缓存中读取，因此需要额外通知数据
    update?: boolean; // 是否需要异步跟新
    updatedCallback?: (_data: any) => void; // 异步更新成功是否需要回调
}): Promise<any> {
    // 给key增加前缀，以防被覆盖
    const key = 'CACHE_' + info.key;
    let data;
    if (info.cache == false) {
        data = null; // 如果主动设置了 cache 是 false，那么表明不使用缓存
    } else {
        data = getExpiredData(key, info.isLocal || false);
    }
    // data = null; // TODO 测试阶段，关闭缓存
    if (data) {
        if (info.notice) info.notice(true);
        if (info.update === true) {
            setTimeout(async () => {
                const d = await info.execute();
                if (data.Ok !== undefined) {
                    setExpiredData(key, d, info.ttl || 60 * 60, info.isLocal || false);
                    if (info.updatedCallback) info.updatedCallback(d);
                }
            }, 0);
        }
        return data;
    } else {
        const timeout = info.timeout ?? 30000; // 默认 30 秒
        // 如果超时就返回错误
        data = await new Promise((resolve, reject) => {
            const start = new Date().getTime();
            let flag = false;
            const timeoutCallback = () => {
                ElMessage.error(t('message.error.network.timeout'));
                reject('timeout');
            };
            setTimeout(() => !flag && timeoutCallback(), timeout);
            info.execute()
                .then((d) => {
                    if (new Date().getTime() <= start + timeout) {
                        resolve(d);
                    } else {
                        timeoutCallback(); // 即使获取到内容，超时了，也不接受
                    }
                })
                .catch((e) => reject(e))
                .finally(() => (flag = true));
        });

        // 缓存中没有就执行方法产生最新的值
        data = await info.execute();
        // console.log('execute result for ' + key, data);
        // 如果数据没有正常返回，就不缓存了
        if (data === undefined) {
            if (info.notice) info.notice(false);
            return data;
        }
        setExpiredData(key, data, info.ttl || 60 * 60, info.isLocal || false);

        if (info.notice) info.notice(false);
        return data;
    }
}

type CacheItem = {
    value: any;
    expired: number; // 毫秒过期时间
};

//内存变量
const CACHE_DATA = {};

// 保存值到缓存
// key: 设置storage里的key
// value: 传入要存储的值
// ttl: Time To Live 过期时长，秒为单位，比如xx秒以后过期
// isLocal: true为LocalStorage，false为非LocalStorage
const setExpiredData = (key: string, value: any, ttl: number, isLocal: boolean): void => {
    const now = new Date().getTime();
    // 1. 加上过期时间
    const item: CacheItem = {
        value: value,
        expired: now + ttl * 1000,
    };
    // 2. 将数据存在内存变量里，以便不用每次从 localStorage 中读取
    CACHE_DATA[key] = item;

    //定义即将存入localStorage里的对象中每个value的替换方法，setItem时使用
    function replacer(key, value) {
        // console.log("value",key,value,typeof value)
        if (typeof value === "bigint") {
            return Number(value);
        } else if (typeof value === "object" && value?.constructor.name === 'Principal') {
            // console.log("value object", key, value)
            // console.log("value object", typeof value);
            // 注意，value为null时，type为object
            // 将Principal格式的value转换为字符串储存
            // 以免JSON.stringify深拷贝时破坏Principal的constructor，导致无法转换成字符串
            return value.toString();
        } else if (value && value._isPrincipal) {
            console.log("value._isPrincipal", key, value)
            console.log("value._isPrincipal", value?.constructor.name)
            return value.toString();
        }
        return value;
    }
    // 3. 如果需要存储至 localStorage
    if (isLocal) {
        localStorage.setItem(
            key,
            JSON.stringify(item, replacer),
        );
    }
};

// 获取包含过期时间和使用次数的localStorage
const getExpiredData = (key: string, isLocal: boolean): any => {
    const now = new Date().getTime();
    // 1. 先读取内存中的值
    let item = CACHE_DATA[key] as CacheItem;
    if (item) {
        // 比对是否过期
        if (item.expired < now) {
            // 已经过期 需要删除
            delete CACHE_DATA[key];
            localStorage.removeItem(key);
            return null;
        } else {
            return item.value;
        }
    }
    if (!isLocal) {
        // 未存本地则不继续找了
        return null;
    }
    // 2. 内存中没有，再读取持久化的信息
    const itemString = localStorage.getItem(key);
    if (!itemString) {
        return null;
    }
    item = JSON.parse(itemString) as CacheItem;
    if (item.expired == undefined || item.expired < now) {
        // 没有过期时间 直接删除
        // 已经过期 需要删除
        delete CACHE_DATA[key];
        localStorage.removeItem(key);
        return null;
    }
    // 2.1 将持久化的信息在内存中存一份
    CACHE_DATA[key] = item;
    return item.value;
};

// 清除指定缓存，有些数据需要更新，有缓存情况下，影响显示
export const clearCacheData = (key: string): void => {
    key = 'CACHE_' + key;
    console.error('clear cache data for key: ' + key);
    delete CACHE_DATA[key];
    localStorage.removeItem(key);
};
