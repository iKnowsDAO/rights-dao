// 统一要上传图片的结构
export interface PictureInfo {
    //上传图片接口里的content是readPhotoUpload方法里返回的buffer
    content: Array<number>;
    //fileType就是type
    fileType: string;
}
