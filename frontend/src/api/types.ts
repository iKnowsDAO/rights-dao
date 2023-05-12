import { Principal } from "@dfinity/principal/lib/cjs";

// 后端的错误
export type ApiError = {
    userAlreadyExists: string; // 在用户已经注册的情况下，返回的用户名
    unauthorized: null; // 未注册情况下，调用接口
    emailAddressAlreadyExists: null; // 邮箱已经注册
    emailAddressNotValid: null; // 邮箱格式不正确
    UserNotFound: null; // 找不到目标用户
    UserEmailInvalid: null; // 用户的邮箱格式存在问题
    PostAlreadyCompleted: null; //贴子已经进入完成状态，不能再修改
    PostNotFound: null; // 贴子找不到

    ProposalNotFound: null;//提案找不到
    ProposalUnAuthorized: null;
    ProposalDeadlineOutOfDate: null;
    MemberNotFound: null;
    MemberAlreadyExists: null;
    MemberPrincipalWrongFormat: null;
    AnswerWithCommentCantDelete: null; //有评论的回答不能删除
};

// 后端的返回结果
export type ApiResult<T> = {
    Ok?: T;
    Err?: ApiError;
};
// 后端的返回结果 分页
export type ApiResultByPage<T> = {
    Ok?: {
        data: T[];
        page_num: bigint; // 当前页码
        page_size: bigint; // 页面大小
        total_count: bigint; // 总数
    };
    Err?: ApiError;
};

export type ApiStatus = 'enable' | 'disable' | 'pending' | 'deleted';
export type ApiPostStatus = {
    Enable?: null,
    Completed?: null,
    Closed?: null,
};// 正常 | 完成 | 关闭

export type ApiDaoState = {
    Open?: null, //投票中
    Accepted?: null, //yes的票数足够，已通过，待执行
    Rejected?: null, //no的票数足够，未通过，不会执行
    Executing?: null, //正在执行
    Succeeded?: null, //成功执行
    Failed?: string, //发生意外的故障，并且包含失败原因
};

export type RichText = {
    content: string; // 实际内容
    format: 'text' | 'markdown' | 'html'; // 标记内容类型 有 3 种: text | html | markdown
};

export type PostCategory = {
    Tech?: null,
    Law?: null,
    Safeguard?: null, //维权
    Blacklist?: null, //黑名单
    Other?: null
};

export type ApiUserInfo = {
    id: bigint; //id
    owner: Principal | string; // 用户principal，唯一。从后端接受时是principal格式，再通过toString显示成字符串格式。
    email: string;
    name: string;
    memo: string;
    status: string;
    created_at: bigint;
    avatar_uri: string; //头像网址，暂时没用
    location: string; // 位置，用户想写就写。限制30个字
    avatar_id: bigint; //头像id，暂时没用
    biography: string; //类似于个人签名
    interests: string[]; //兴趣，类似于标签
    wallet_principal: string[]; //用户绑定的钱包
    achievement: [AchievementResult];//成就完成详情
    claimed_sbt: [ClaimedMedalMeta] //收集的SBT，opt格式，只会有一个。
};

export type ApiPost = {
    id: bigint; //id
    author: Principal | string; // 作者
    authorData?: ApiUserInfo; //作者详细资料 通过id请求对应方法，获取对应资料
    title: string;
    comments?: ApiPostComments[];
    comment_count: number[]; //opt格式的回复数
    content: RichText;
    category: PostCategory;
    photos: number[];
    answer: number[];
    participants: string[];//期待参与的人
    end_time: [number]; //结束时间  opt格式，类似于[1000]，数组中只有一个数据。
    bounty_sum: [number]; //问题的赏金 opt格式
    events?: ApiPostTimeline[];
    likes_count: number; //点赞数
    ask_for_money: any;
    status: ApiPostStatus;
    created_at: number;
    updated_at: number;
};

export type ApiPostTimeline = {
    author: Principal | string; // 作者
    created_at: number;
    description: string;
    event_time: number;
    type?: string;
    hollow?: boolean;
    time?: string;
}

export type ApiPostComments = {
    author: Principal | string; // 作者principalID
    authorData?: ApiUserInfo; //作者详细资料 通过id获取对应资料
    comment_id: number[];
    comments: [];
    quote_id: number[];
    quoteName?: string;
    content: RichText;
    created_at: bigint;
    id: bigint;
    post_id: bigint;
    isReply?: boolean; //是否被回复
    likes_count: number;
}

export type ApiProfilePost = {
    author?: Principal | string; // 作者principalID
    content: RichText;
    comments?: [];
    comment_count?: number[]; //opt格式的回复数
    length?: number;
    created_at: bigint;
    title: string;
    post_title?: string; //评论的title名
    id: bigint;
    post_id?: bigint;
}

export type UserReputation = {
    user: Principal | string; // 用户principalID
    amount: bigint; //积分值
}
//DAO成员信息
export type GovernanceMember = {
    user: Principal; // 用户principalID
    created_at: bigint; //创建时间
}

export type Weights = {
    amount: number;
}
export type Execute_args = {
    execute_args: execute_args;
}
export type execute_args = {
    AddGovernanceMember: ApiDaoArgs;
}
export type ApiDaoProposal = {
    id: bigint; //id
    proposer: Principal | string; // 发起人
    authorData?: ApiUserInfo;  //作者详细资料 通过id获取对应资料
    payload: Execute_args; //额外参数
    state: ApiDaoState;
    votes_yes: Weights;
    votes_no: Weights;
    vote_threshold: Weights; //投票阈值
    voters: Principal[] | string[];
    created_at: number;
};

export type ApiDaoArgs = {
    id: Principal | string; //被提案用户的id
    title: string;
    content: RichText;
    deadline: number;
    action: string; // action目前只有Add,Delete两个值，对应增加删除管理员
};

//勋章等级
export type MedalLevel = {
    Commoner: null, //
    Bronze: null, //青铜
    Silver: null,
    Gold: null,
    Platinum: null,
    Diamond: null
}
//勋章数据表
export type MedalMeta = {
    name: MedalLevel;
    level: number;
    experience: bigint;
    photo_url: string;
};

//用户收集的勋章数据表
export type ClaimedMedalMeta = {
    created_at: number,
    id: number,
    medal: MedalMeta,
    owner: Principal
};

//用户等级
export type ApiUserLevel = {
    owner: Principal;
    level: bigint; //等级
    experience: bigint; //当前经验值
    next_level: string; //升到下一级所需要的经验总量
};

export type ApiAchievement = {
    description: string; //描述
    experience: bigint; // 达成后获得的经验值
    completion: bigint;// 当前完成的次数
    target: bigint;// 完成成就需要完成的次数
    keyword: string; //关键词
    level: MedalLevel;
};

export type AchievementResult = {
    active_user: ApiAchievement, //活跃用户（发贴n次）
    post_comment: ApiAchievement, //问题获得回复
    issued_bounty: ApiAchievement, //给赏金
    received_bounty: ApiAchievement, //收到赏金
    reputation: ApiAchievement, //积分成就
};
