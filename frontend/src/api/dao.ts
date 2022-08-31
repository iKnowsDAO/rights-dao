import {getBackend} from "@/api/canister_pool";
import {ApiResult, RichText} from "@/api/types";

// 增加DAO提案,id是涉及到目标的principalId
// action目前只有Add,Delete两个值，对应增加删除管理员
// deadline限制最短2天，最长7天
export async function addDaoProposal(proposal:any | { id: string, title: string, content: RichText, deadline: number, action: string }):
    Promise<ApiResult<boolean>> {
    return getBackend().submit_add_governance_member_proposal(proposal);
}
