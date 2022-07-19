module.exports = {
    plugins: [
        // 移除打包element时的@charset警告
        // 忽略element的scss字符集，报错: @charset" must be the first rule in the file
        {
            postcssPlugin: 'internal:charset-removal',
            AtRule: {
                charset: (atRule) => {
                    if (atRule.name === 'charset') {
                        atRule.remove();
                    }
                },
            },
        },
    ],
};
