//传入名字,根据名字生成颜色,这样颜色就固定下来了
export function extractColorByName(name: string): string {
    name = name + '123456';
    const temp = ['#'];
    for (let index = 0; index < name.length; index++) {
        temp.push(parseInt(name[index].charCodeAt(0).toString(), 10).toString(16));
    }
    return temp.slice(0, 5).join('').slice(0, 4);
}

// 处理头像名字显示的逻辑
export function showAvatarName(username: string, principalId: string) {
    if (username) {
        //只取首字母大写为头像名字
        return username.substring(0, 1).toUpperCase();
    }
    //没有名字返回principalId首字母作为名字
    return principalId ? principalId.substring(0, 1).toUpperCase() : '';
}
