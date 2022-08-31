import {getCache, TTL} from '@/common/cache';
import {getBackend} from "@/api/canister_pool";
import {ApiPost, ApiPostComments, ApiPostTimeline, ApiResult, ApiResultByPage} from "@/api/types";

// 更新用户自己的信息
export async function submitPost(post: any | ApiPost): Promise<ApiResult<boolean>> {
    return getBackend().create_post(post);
}

// 获取贴子详情，所有内容
// category是opt text形式，传入空数组则查询所有板块内容
export async function getPostPage(pageNum: number, pageSize: number, query: string, category: string[]): Promise<ApiResultByPage<any>> {
    return getBackend().page_posts({page_num: pageNum, page_size: pageSize, querystring: query, category: category});
}

// 获取贴子详情，所有内容
export async function getPost(id: number): Promise<ApiResult<ApiPost>> {
    return getBackend().get_post({id: id});
}

// 删除贴子，只有用户自己可以操作
export async function deletePost(id: number): Promise<ApiResult<boolean>> {
    return getBackend().delete_post({id: id});
}

// 获取贴子详情，不获取时间线和回贴
export async function getPostInfo(id: number): Promise<ApiResult<ApiPost>> {
    return getBackend().get_post_info({id: id});
}

// 获取贴子相关时间线
export async function getPostTimeLine(id: number): Promise<ApiResult<ApiPostTimeline[]>> {
    return getBackend().get_post_events({id: id});
}

// 获取贴子相关回帖
export async function getPostComments(id: number): Promise<ApiResult<ApiPostComments[]>> {
    return getBackend().get_post_comments({id: id});
}

// 增加贴子的时间线
export async function addPostTimeline(timeline: { post_id: number, event_time: number, description: string }): Promise<ApiResult<boolean>> {
    return getBackend().add_post_event(timeline);
}

// 修改贴子状态，后台会顺便更新时间线
export async function changePostStatus(timeline: { id: number, status: string, description: string }): Promise<ApiResult<boolean>> {
    return getBackend().change_post_status(timeline);
}

// 增加贴子的回贴
export async function addPostReply(id: number, content: string): Promise<ApiResult<boolean>> {
    return getBackend().add_post_comment({
        post_id: id,
        content: {
            content: content,
            format: "html"
        }
    });
}

// 增加贴子回答的评论
export async function addPostReplyReply(commentId: number, postId, content: string, quoteId: number[]): Promise<ApiResult<boolean>> {
    return getBackend().add_comment_comment({
        post_id: postId,
        comment_id: commentId,
        quote_id: quoteId,
        content: {
            content: content,
            format: "text" //评论只是单纯的text，不含html
        }
    });
}

// 采纳贴子的最佳答案，只能由提问人调用此方法。
export async function submitPostAnswer(postId: number, commentId: number): Promise<ApiResult<boolean>> {
    return getBackend().submit_post_answer({
        post_id: postId,
        comment_id: commentId,
    });
}
