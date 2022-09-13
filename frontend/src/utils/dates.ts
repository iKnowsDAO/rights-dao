import {SupportedLocale, t} from '@/locale';
import i18n from "@/locale";

// 英文月份
const TOTAL_MONTHS = [
    'January',
    'February',
    'March',
    'April',
    'May',
    'June',
    'July',
    'August',
    'September',
    'October',
    'November',
    'December',
];

// 字符左边补 0
const leftPaddingZero = (value: string, length: number): string => {
    while (value.length < length) {
        value = '0' + value;
    }
    return value;
};
// 下边这个方法是判断时间是否小于10，小于10的数字 前边加0
export const isAddZero = (time: number) => {
    return time < 10 ? '0' + time : time.toString();
}

//将纳秒的时间戳转换为毫秒的日期类型，方便前端数据回显
export const parseNs2Date = (time: number): Date => {
    return new Date(time / (1000 * 1000));
};

//直接将后端时间戳转换成yyyy-mm-dd格式的字符串
export const formatDate = (value: bigint | number): string => {
    const valueNumber = Number(value);
    let date = new Date(valueNumber / (1000 * 1000))
    let y = date.getFullYear()  //获取年份
    let m = date.getMonth() + 1  //获取月份
    let month = m < 10 ? "0" + m : m  //月份不满10天显示前加0
    let d = date.getDate()  //获取日期
    const day = d < 10 ? "0" + d : d  //日期不满10天显示前加0
    return y + "-" + month + "-" + day
};
//直接将后端时间戳转换成yyyy-mm-dd HH:MM格式的字符串
export const formatDateToMinutes = (value: bigint | number): string => {
    const valueNumber = Number(value);
    let date = new Date(valueNumber / (1000 * 1000));
    let y = date.getFullYear();  //获取年份
    let m = isAddZero(date.getMonth() + 1);  //获取月份
    let d = isAddZero(date.getDate());  //获取日期
    const H = isAddZero(date.getHours()); // 小时
    const M = isAddZero(date.getMinutes()); // 分钟
    return y + "-" + m + "-" + d + " " + H + ":" + M
};

//将后端时间戳转换成友好显示格式的字符串
export const getTimeF = (value: bigint | number): string => {
    //不管是bigint 还是 number 先转换为number方便计算
    const valueNumber = Number(value);
    const minute = 1000 * 60;
    const hour = minute * 60;
    const day = hour * 24;
    const week = day * 7;
    const month = day * 30;
    //将纳秒时间戳转换为正常的毫秒时间戳
    let date = new Date(valueNumber / (1000 * 1000))
    const time1 = new Date().getTime();//当前的时间戳
    const time2 = date.getTime();
    //时间差 60000 1分钟 //3600000 1小时 //86400000 24小时 //对传入时间进行时间转换
    const time = time1 - time2;
    let result = "";
    //太久了就直接显示日期吧
    //time为负数，说明是之后的时间，暂时直接显示日期
    if (time < 0) {
        result = formatDate(valueNumber);
        // result = "发布于" + parseInt(time / month + "") + "月前！";
    } else if (time / month >= 1) {
        result = formatDate(valueNumber);
        // result = "发布于" + parseInt(time / month + "") + "月前！";
    } else if (time / week >= 1) {
        result = formatDate(valueNumber);
        // result = "发布于" + parseInt(time / week + "") + "周前！";
    } else if (time / day >= 1) {
        const count = parseInt(time / day + "")
        result = t('date.post') + i18n.global.tc('date.day', count);
    } else if (time / hour >= 1) {
        const count = parseInt(time / hour + "")
        result = t('date.post') + i18n.global.tc('date.hour', count);
    } else if (time / minute >= 1) {
        const count = parseInt(time / minute + "")
        result = t('date.post') + i18n.global.tc('date.minute', count);
    } else {
        result = t('date.just');
    }
    return result;
};


// 根据语言选择格式化日期，显示顺序是日期 月份 年份
export const formatDateWithTotalMonth = (locale: SupportedLocale, date: Date): string => {
    let year = '';
    let month = '';
    let day = '';
    switch (locale) {
        case SupportedLocale.zhCN:
            // 中文要特别处理一下
            year = date.getFullYear().toString();
            month = (date.getMonth() + 1).toString();
            day = date.getDate().toString();
            return year + '年' + leftPaddingZero(month, 2) + '月' + leftPaddingZero(day, 2) + '日';
        default:
    }
    // 默认英文日期
    return date.getDate() + '  ' + TOTAL_MONTHS[date.getMonth()] + '  ' + date.getFullYear();
};
