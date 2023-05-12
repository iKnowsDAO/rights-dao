// 将一个数字格式化带逗号形式的字符串
export function toThousands(
    num: number, // 要格式化的数字
    keep: number, // 需要的有效位数 如果是 0 表示全要
): { value: string; lost: boolean; join: string } {
    let value = '';
    let lost = false;

    const addComma = (n: string, direction: 'right' | 'left'): string => {
        const result: string[] = [];
        let count = 0;
        const ns = (n || 0).toString().split('');
        if (direction === 'right') {
            // 整数从右边开始数
            for (let i = ns.length - 1; 0 <= i; i--) {
                count++;
                result.unshift(ns[i]);
                if (!(count % 3) && i != 0) result.unshift(',');
            }
        } else {
            // 小数从左边开始数
            for (let i = 0; i < ns.length; i++) {
                count++;
                result.push(ns[i]);
                if (!(count % 3) && i != ns.length - 1) result.push(',');
            }
        }
        return result.join('');
    };

    const num_str = num.toString();
    if (num_str.indexOf('e') !== -1)
        // 不处理科学计数的数字
        return { value: num_str, lost: false, join: num_str };
    const index = num_str.indexOf('.');

    // 区分整数部分和小数部分
    const num1 = index === -1 ? num_str : num_str.substring(0, index);
    const num2 = index === -1 ? '' : num_str.substring(index + 1);
    if (keep === 0) keep = index === -1 ? num1.length : num1.length + num2.length;

    // 整数部分
    if (num1.length > keep) {
        const base = Math.pow(10, num1.toString().length - keep);
        const num11 = Math.floor(parseInt(num1) / base) * base;
        if (num > num11) lost = true;
        value = addComma(num11.toString(), 'right');
    } else if (num1.toString().length === keep) {
        if (num2) lost = true;
        value = addComma(num1, 'right');
    } else {
        value = addComma(num1, 'right'); // 先把整数部分处理好
        // 存在整数位数为 0 的情况
        const num1_length = num1 === '0' ? 0 : num1.length;
        if (num2) {
            // 如果有小数部分
            let scale = keep - num1_length;
            let base = 1;
            while (scale > 0) {
                base /= 10;
                scale--;
            }
            if (parseFloat('0.' + num2) < base) {
                // 小数部分太小 全部丢弃
                lost = true;
                if (num1 === '0') {
                    value = num.toString();
                    lost = false;
                }
            } else {
                scale = keep - num1_length;
                // 截取保留位数的数字
                const num22 = num2.length > scale ? num2.substring(0, scale) : num2;
                if (parseFloat('0.' + num2) > parseFloat('0.' + num22)) lost = true;
                value = value + '.' + addComma(num22, 'left');
            }
        }
    }
    return { value, lost, join: value + (lost ? '+' : '') };
}

// 获取一个随机的 32 位无符号整数
export function getRandomUint32(): number {
    const next = () => (Math.random() >= 0.5 ? '1' : '0');
    let value = '';
    for (let i = 0; i < 32; i++) value += next();
    // console.log('getRandomUint32', value);
    return parseInt(value, 2);
}

// 获取一个随机的 64 位 无符号整数 好像有 bug
export function getRandomUint64(): number {
    return Number(BigInt(getRandomUint32()) * BigInt(getRandomUint32()));
}

// 将目标数值和现有数值转化为百分比（保留2位小数），满100%则计算为100%
export function calculatePercent(currentValue: number | bigint, targetValue: number | bigint): number {
    currentValue = Number(currentValue);
    targetValue = Number(targetValue);
    const percent = (currentValue / targetValue) * 100;
    return Number((percent >= 100 ? 100 : percent).toFixed(2));
}
