import {getUserIsAdmin} from "@/api/user";
import {getCache, TTL} from "@/common/cache";
import {getBackend, getCurrentPrincipal} from "@/api/canister_pool";
import {Principal} from "@dfinity/principal/lib/cjs";

// 鉴别管理员
export async function showAdmin() {
    //如果没有登录，直接返回false
    if (!getCurrentPrincipal()) {
        return false;
    }
    const res = await getCache({
        key: 'Admin_' + getCurrentPrincipal().toUpperCase(),
        execute: () => getUserIsAdmin(getCurrentPrincipal()),
        ttl: TTL.minute30,
        isLocal: true, // 需要本地存储
    });
    if (res.Ok && res.Ok.id.toString() === getCurrentPrincipal()) {
        return true;
    } else {
        return false;
    }
}

// 查询是否有管理员cookie
export function isUserAdmin() {
    const isAdmin = sessionStorage.getItem("isAdmin");
    console.log("isAdmin",isAdmin)
    //如果存在值则返回true
    return !!isAdmin;
}

