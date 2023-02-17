import {getCache, TTL} from '@/common/cache';
import {getBackend} from "@/api/canister_pool";
import {ApiPost, ApiPostComments, ApiPostTimeline, ApiResult, ApiResultByPage} from "@/api/types";

// 发贴
export async function submitPost(post: any | ApiPost): Promise<ApiResult<boolean>> {
    return getBackend().create_post(post);
}

// 获取贴子详情，所有内容
// category是opt text形式，传入空数组则查询所有板块内容
export async function getPostPage(pageNum: number, pageSize: number, query: string, category: string[]):
    Promise<ApiResultByPage<ApiPost>> {
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

// 删除贴子的回答，只有管理员可以操作
export async function deletePostAnswer(post_id: number, answer_id: number): Promise<ApiResult<boolean>> {
    return getBackend().delete_post_answer({post_id: post_id, answer_id: answer_id});
}

// 删除贴子的回答的评论，只有管理员可以操作
export async function deletePostComment(post_id: number, answer_id: number, comment_id: number): Promise<ApiResult<boolean>> {
    return getBackend().delete_post_answer_comment({post_id: post_id, answer_id: answer_id, comment_id: comment_id});
}

// 获取贴子详情，不获取时间线和回贴
export async function getPostInfo(postId: number): Promise<ApiResult<ApiPost>> {
    return getBackend().get_post_info({id: postId});
}

// 获取贴子相关时间线
export async function getPostTimeLine(postId: number): Promise<ApiResult<ApiPostTimeline[]>> {
    return getBackend().get_post_events({id: postId});
}

// 获取贴子相关回帖
export async function getPostComments(postId: number): Promise<ApiResult<ApiPostComments[]>> {
    return getBackend().get_post_comments({id: postId});
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
        answer_id: commentId,
    });
}

// 点赞贴子或者回答
export async function likePost(postId: number, isPost: boolean, commentId?: number): Promise<ApiResult<boolean>> {
    if (isPost) {
        //是问题
        return getBackend().like_post({
            post_id: postId,
        });
    } else {
        //是回答。
        return getBackend().like_post_answer({
            post_id: postId,
            comment_id: commentId,
        });
    }
}

// 取消点赞贴子或者回答
export async function cancelLike(postId: number, isPost: boolean, commentId?: number): Promise<ApiResult<boolean>> {
    if (isPost) {
        //是问题
        return getBackend().cancel_like_post({
            post_id: postId,
        });
    } else {
        //是回答
        return getBackend().cancel_like_post_answer({
            post_id: postId,
            comment_id: commentId,
        });
    }
}

// 是否点赞过问题或回答
export async function isLikedPost(postId: number, isPost: boolean, commentId?: number): Promise<ApiResult<boolean>> {
    if (isPost) {
        return getBackend().is_like_post({
            post_id: postId,
        });
    } else {
        //是回答
        return getBackend().is_like_post_answer({
            post_id: postId,
            comment_id: commentId,
        });
    }
}

// 获得3个点赞数最高的回答
export async function getTopLikePosts(): Promise<ApiResult<ApiPost[]>> {
    return getBackend().get_top_likes_posts();
}
