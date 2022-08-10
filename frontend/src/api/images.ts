import {PictureInfo} from '@/types/picture';
import {getCache, TTL} from '@/common/cache';
import {getPhoto} from './canister_pool';
import {showMessageError} from "@/utils/message";
import {t} from '@/locale';

export async function savePic(
    picture: PictureInfo,
    picName: string,
    picDescription: string,
    userPrincipal: string | undefined,
): Promise<any> {
    //图片名字和描述传空字符串省略
    return await getPhoto().savePic(picture, picName, picDescription, userPrincipal).catch(e => {
        console.error("savePic", e)
        showMessageError(t("message.error.canister"))
    });
}

//根据图片id，获取直接保存的图片信息（很少用，正常情况应该使用getUrlByPhotoServiceId方法，通过网址访问图片）
export async function getPic(pictureId: number): Promise<any> {
    // return getPhoto().getPictureProfile(pictureId);
    return await getCache({
        key: 'GET_PICTURE_' + pictureId,
        execute: () => getPhoto().getPictureProfile(pictureId),
        ttl: TTL.day1, // 图片缓存1天
        isLocal: false, // 图片不需要本地存储
    });
}
