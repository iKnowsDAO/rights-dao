export default {
    proposal: "DAO 提案",
    create: "创建提案",
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
    information: {
        title: "信息",
        action: {
            label: "提案类型",
            add: "增加管理员",
            remove: "移除管理员"
        },
        target: "提案目标",
        start: "开始日期",
        end: "结束日期",
        threshold: {
            title: "投票阈值",
            tips: "当投票数达到阈值时，提案会自动执行"
        },
    },
    vote: {
        title: "投出您的票",
        dialogTitle: "投票确认",
        yes: "您确认要支持 {id} 号提案吗？",
        no: "您确认要反对 {id} 号提案吗？",
    },
    result: {
        title: "结果",
        yes: "支持",
        no: "反对",
    },
    state: {
        open: "进行中",
        accepted: "已通过",
        rejected: "未通过",
        executing: "执行中",
        succeeded: "已成功",
        failed: "意外失败",
    }
};
