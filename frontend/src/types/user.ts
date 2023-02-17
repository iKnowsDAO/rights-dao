// 统一用户信息的结构
// 注意和types.ts中的ApiUserInfo统一格式
export class UserInfo {
    id = 0; //用户id
    owner = ''; // 用户principal，唯一
    email = ''; //邮箱
    name = ''; // 用户自己设置的用户名
    biography = ''; // 用户签名
    status = ''; //用户状态
    create_at = 0; //注册时间
    avatar_id = 0; // 头像 id
    wallet_principal = ['']; // 用户绑定钱包
}

export interface UserInfoElement {
    owner?: string;
    name?: string;
    wallet_principal?: string[];
    avatarId?: number;
}
