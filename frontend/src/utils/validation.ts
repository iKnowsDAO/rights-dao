// 校验邮箱
export function validateEmail(email: string): { result?: string; error?: string } {
    if (!email) return {};
    email = email.trim();
    // 1. 不能包含空格 是否以字母结束
    if (email.indexOf(' ') >= 0) return {};
    if (!/[a-zA-Z]$/.test(email)) return {};
    // 2. 字符串长度大于等于5，其中用户名不能超过20个字符，域名也不能超过20个字符
    // 3. 必须有且只有一个@（电子邮件地址标志符号）
    const ss = email.split('@');
    if (ss.length != 2) return {};
    if (ss[1].length > 20) return {};
    // 4. 用户名（@前面的字符串）是字符或数字，或_(下划线)
    // 5. 域名只能是(@后面的字符串)是是字符或数字，或_(下划线)
    if (!/^[A-Za-z0-9_]{5,20}@[a-zA-Z0-9_]+(\.[a-zA-Z0-9_]+)*(\.[a-zA-Z]+){1}$/.test(email))
        return { error: 'message.subscribe.email.tipFormat' };
    return { result: email };
}
