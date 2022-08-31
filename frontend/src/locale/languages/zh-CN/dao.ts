export default {
    proposal: "DAO 提案",
    form: {
        title: {
            label: "提案标题：",
            placeholder: "请输入提案标题",
        },
        content: {
            label: "提案内容：",
            placeholder: "请输入提案内容",
        },
        action: {
            label: "提案类型：",
            placeholder: "请选择提案类型",
            addAdmin: "增加管理员",
            tips: "每次提案只能有一名被提案人选"
        },
        related: {
            label: "提案相关人：",
            placeholder: "请输入提案相关人的principalId",
            add: "增加提案相关人"
        },
        deadline: {
            tips: "最短提案结束时间为2天，最长为7天"
        }
    },
};
