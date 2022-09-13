import {getBackend} from "@/api/canister_pool";
import {ApiDaoArgs, ApiDaoProposal, ApiResult, ApiResultByPage} from "@/api/types";

// 增加DAO提案,id是涉及到目标的principalId
// action目前只有Add,Delete两个值，对应增加删除管理员
// deadline限制最短2天，最长7天
export async function addDaoProposal(proposal: any | ApiDaoArgs):
    Promise<ApiResult<boolean>> {
    return getBackend().submit_add_governance_member_proposal(proposal);
}

//获取dao提案详情, 10014n
export async function getDaoProposal(id: number): Promise<ApiResult<ApiDaoProposal>> {
    return getBackend().get_governance_proposal({id: id});
}

// 给提案投票
export async function voteProposal(proposalId: number, vote): Promise<ApiResult<boolean>> {
    console.log("vote",{
        proposal_id: proposalId,
        vote: vote
    })
    return getBackend().vote_governance_proposal({
        proposal_id: proposalId,
        vote: vote
    });
}

//分页查询dao提案
export async function getPageDaoProposal(pageNum: number, pageSize: number, query: string):
    Promise<ApiResultByPage<ApiDaoProposal>> {
    return getBackend().page_governance_proposals({page_num: pageNum, page_size: pageSize, querystring: query});
}
