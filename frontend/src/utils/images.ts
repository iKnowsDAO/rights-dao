import {ElMessage} from 'element-plus/es';
import {PictureInfo} from "@/types/picture";
const photoServiceCanisterId = process.env['PHOTO_SERVICE_CANISTER_ID'] ?? '';

//通过http请求的方式获取图片，只需要返回网址赋予给图片标签的src即可显示图片
export function getUrlByPhotoServiceId(id: number | bigint) {
    id = Number(id);
    if (process.env['configMode'] === 'development') {
        //本地调试用本地图片
        const base_url = 'http://localhost:8000';
        return photoServiceCanisterId
            ? `${base_url}?canisterId=${photoServiceCanisterId}&picId=${id}`
            : '';
    }
    return photoServiceCanisterId
        ? `https://${photoServiceCanisterId}.raw.ic0.app/?picId=${id}`
        : '';
}

//计算富文本编辑器中的字数长度
export function calculatedICPIdLength(html): number {
    if (html) {
        //匹配编辑器中的img标签
        const imgReg = /<img.*?(?:>|\/>)/gi;
        //匹配img标签中src含有http的外链
        const httpReg = /src="http[\'\"]?([^\'\"]*)[\'\"]?/i;
        const arr = html.match(imgReg); //筛选出所有的img
        let length = 0;//计算此html字符串实际传到后端的长度
        let lengthHtml = html.toString();//用来计算html长度的备份html
        if (arr && arr.length) {
            for (let i = 0; i < arr.length; i++) {
                //筛选从其他网站直接复制过来的，src是外链的图片标签
                const http = arr[i].match(httpReg);
                //计算长度的lengthHtml把对应上传的图片标签置空，方便计算长度
                lengthHtml = lengthHtml.replace(arr[i], "");
                //存在外链图片，将此外链图片标签跳过上传到IC的步骤。
                if (http) {
                    //增加此标签的长度，保证返回的长度正常
                    length = length + arr[i].length;
                } else {
                    //不存在外链图片，说明是上传的图片，给length直接加长度
                    //例如<icpl.imgId.10028/>的长度
                    length = length + 19;
                }
            }
        }
        length = length + lengthHtml.length;
        return length;
    } else {
        return 0;
    }
}


type htmlItem = {
    overLimitLength?: boolean; // 此html字符串实际传到后端的长度是否超过限制长度
    haveImg: boolean;// 是否不存在图片
    html?: string; //html元素
};
