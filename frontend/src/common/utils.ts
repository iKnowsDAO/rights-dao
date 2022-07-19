
// 统一处理显示用户名的逻辑
export function showUsername(username: string, principalId: string): string {
    if (username) {
        const MAX_LENGTH = 15; // 显示的最长字符
        if (username.length >= MAX_LENGTH) {
            return username.substring(0, MAX_LENGTH - 3) + '...';
        }
        return username;
    }
    return principalId ? principalId.substring(0, 5).toUpperCase() + '...' : '';
}

// 移除html标签，只保留文本内容
export function cleanHtml(html: string): string {
    // 正则里<表示尖括号；
    // 第一个\/?表示类似这种标签的情况；
    // .+?表示将中间的内容替代；
    // 第二个\/?表示<img/>的情况；
    // /g表示全局替换；
    return html.replace(/<\/?.+?\/?>/g,'');;
}
