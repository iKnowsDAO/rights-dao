import { ApiResult } from "@/api/types";
import { getBackend } from "@/api/canister_pool";

const multiply = 10000 * 10000; //10的8次方
//转换位数，乘法
export const SumMultiplier = (amount: number | bigint): number => {
    const type = typeof amount;
    if (type === 'number' || type === 'bigint') {
        return parseInt((Number(amount) * multiply).toString());
    } else {
        return 0;
    }
}
//转换位数，除法
export const SumDivision = (amount: number | bigint): number => {
    const type = typeof amount;
    if (type === 'number' || type === 'bigint') {
        return Number(amount) / multiply;
    } else {
        return 0;
    }
}
//为问题添加待支付的赏金，返回支付单id。
//nonce是前端创建的随机数
export async function addPostBounty(post_id: number, amount: number, nonce: number): Promise<ApiResult<number>> {
    return getBackend().add_post_bounty({
        post_id: post_id,
        amount: SumMultiplier(amount),
        nonce: nonce
    });
}

//完成支付后，更新赏金状态
export async function updatePostBounty(bounty_id: number, amount: number, nonce: number): Promise<ApiResult<boolean>> {
    return getBackend().update_post_bounty({
        bounty_id: bounty_id,
        amount: SumMultiplier(amount),
        nonce: nonce
    });
}
