import { showMessageError } from "@/utils/message";
import { t } from '@/locale';
import { savePic } from "@/api/images";
import { getSBTInfo } from "@/api/user";

const photoServiceCanisterId = process.env['PHOTO_CANISTER_ID'] ?? '';

//通过http请求的方式获取图片，只需要返回网址赋予给图片标签的src即可显示图片
export const getUrlByPhotoServiceId = (id: number | bigint) => {
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

//通过指定等级，返回对应勋章的图片url
export const getSBT = async (level: number | bigint): Promise<string> => {
    const sbtInfo = await getSBTInfo();
    // console.log("sbtInfo",sbtInfo)
    // console.log("level",level)
    const matchedMedal = sbtInfo.find(medal => Number(medal.level) === Number(level));
    // console.log("matchedMedal",matchedMedal)
    if (matchedMedal) {
        return matchedMedal.photo_url;
    } else {
        throw new Error(`No SBT medal found for level ${level}`);
    }
}

// 上传图片
export const uploadImage = async (file: any) => {
    //先检查格式和大小
    const isOk = await beforePictureUpload(file);
    if (isOk) {
        const {buffer, src} = await readPictureForUpload(file);
        // 计算要保存的图片名字
        let name = file.name;
        const index = name.lastIndexOf('.');
        let extend = index >= 0 ? name.substring(index) : ''; // 从名字取
        if (!extend) extend = '.' + file.type.split('/')[1]; // 从文件类型取
        name = file.uid + extend;
        const r = await savePic(
            {
                content: buffer,
                fileType: file.type,
            },
            name,
            '',
            '',
        );
        if (r.ok) {
            console.log("upload ok", r)
            // 直接返回图片url
            return getUrlByPhotoServiceId(r.ok);
        } else {
            console.error("imageUpload failed:", r)
            return "";
        }
    } else {
        return "";
    }
};

// 上传图片检查格式和大小
export const beforePictureUpload = (file: any): boolean => {
    //限制上传的图片格式
    const types = ["image/jpg", "image/png", "image/jpeg"];
    //size单位为M，例如2，则为限制上传2M以内的图片
    const size = 2;
    const isJpgOrPng = types.includes(file.type);
    if (!isJpgOrPng) {
        showMessageError(t('message.error.image.format'));
    }
    const isLtSize = file.size / 1024 / 1024 < size;
    if (!isLtSize) {
        showMessageError(t('message.error.image.size'))
    }
    return isJpgOrPng && isLtSize;
};

//图片上传前，执行上传方法，将图片转化为二进制存储在数据中
export async function readPictureForUpload(
    file: any,
): Promise<{ buffer: Array<number>; src: string }> {
    return new Promise(function (resolve, reject) {
        const reader = new FileReader();
        reader.readAsArrayBuffer(file);
        reader.onload = (e) => {
            if (e.target) {
                const buffer = Array.from(new Uint8Array(e.target.result as ArrayBuffer));
                const src = transformArrayBufferToBase64({
                    content: buffer,
                    fileType: file.type,
                });
                resolve({buffer, src});
            } else {
                reject(e);
            }
        };
    });
}

export function transformArrayBufferToBase64(buffer: {
    content: Array<number>;
    fileType: string;
}): string {
    let binary = '';
    const bytes = new Uint8Array(buffer.content);
    for (let len = bytes.byteLength, i = 0; i < len; i++) {
        binary += String.fromCharCode(bytes[i]);
    }
    return `data:${buffer.fileType};base64,` + window.btoa(binary);
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
                    //例如<icp.imgId.10028/>的长度
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
