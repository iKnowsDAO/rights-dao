
import Hash "mo:base/Hash";
import Trie "mo:base/Trie";

module {
    public type Picture = {
        content: Blob;
        fileType: Text;
    };  
    
    public type Timestamp = Int;    
    public type PictureId = Nat;
    public type UserPrincipal = Text;
    public type PictureStore = Trie.Trie<PictureId, PictureProfile>;
    
    public type PictureKey = Trie.Key<PictureId>;

    /// 图片的基本信息
    public type PictureProfile = {
        id : PictureId;
        pic:  Picture;
        picName: Text;
        description: Text;
        owner: UserPrincipal;
        status: PictureStatus;
        createdBy: UserPrincipal;
        createdAt: Timestamp;
    };

    public type PictureStatus = {
        #enable;
        #disable;
        #pending;
    };

    
    public type PicError = {
        #picNotFound;
        #picAlreadyExisted;
        #tooLargeQuantity;
    };

    public type PictureCreateReq = {
        pic: Picture;
        picName: Text;
        description: Text;
    };

    /// 外部请求数据
    public type PictureEditReq = {
        id: Nat;
        pic: Picture;
        picName: Text;
        description: Text;
        owner: Text;
        status: Text;
    };

    /// 辅助方法 把外部请求的修改图片数据转成PictureProfile格式,其中图片状态文本格式需要转换
    /// Args:
    ///     |req|   带图片数据的请求
    ///     |editer|    编辑图片的操作者
    ///     |editTime|  编辑图片的时间
    /// Returns:
    ///     转换成功后的PictureProfile
    public func editReqToPicture(
        req: PictureEditReq,
        editer: UserPrincipal,
        editTime: Int
        ) : PictureProfile {
            {
                id = req.id;
                pic = req.pic;
                picName = req.picName;
                description = req.description;
                owner = req.owner;
                status = textToPictureStatus(req.status);
                createdAt = editTime;
                createdBy = editer
            }
        };

    /// 辅助方法 把文本转换为PictureStatus
    /// Args:
    ///     |text_status|   需要转成PictureStatus枚举的文本: enable, disable, pending
    /// Returns:
    ///     返回文本对应的PictureStatus定义的枚举，enable -> #enable, disable -> #disable, _ -> #pending
    public func textToPictureStatus(text_status: Text) : PictureStatus {
        if (text_status == "enable") {
            #enable
        } else if (text_status == "disable") {
            #disable
        } else {
            #pending
        }
    };

    /// 辅助方法,判断两张图片信息是否一致
    public func pictureEq(lhs: PictureId, rhs: PictureId) : Bool {
        lhs == rhs
    };

    public func pictureMapFilterFn(picId: PictureId, picProifle: PictureProfile) : ?PictureId {
        ?picId
    };

    /// 辅助方法，Trie.find方法乃至的Trie.Key实例
    public func keyOfPicture(picId: PictureId): Trie.Key<PictureId> {
        { key = picId; hash = Hash.hash(picId) }
    };

    public type Page<T> = {
        data : [T];
        pageSize : Nat;
        pageNum : Nat;
        totalCount: Nat;
    };
};