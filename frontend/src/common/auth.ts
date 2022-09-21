import {getUserIsAdmin} from "@/api/user";
import {getCache, TTL} from "@/common/cache";
import {getBackend, getCurrentPrincipal} from "@/api/canister_pool";

// 鉴别管理员
export async function showAdmin() {
    return await getCache({
        key: 'IsAdmin_' + getCurrentPrincipal().toUpperCase(),
        execute: () => getUserIsAdmin(getCurrentPrincipal()),
        ttl: TTL.minute30,
        isLocal: true, // 需要本地存储
    });
}

// 查询是否有管理员cookie
export function isUserAdmin() {
    const isAdmin = sessionStorage.getItem("isAdmin");
    console.log("isAdmin",isAdmin)
    //如果存在值则返回true
    return !!isAdmin;
}

