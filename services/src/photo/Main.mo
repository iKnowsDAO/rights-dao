

import Debug "mo:base/Debug";
import Nat "mo:base/Nat";
import Principal "mo:base/Principal";
import Result "mo:base/Result";
import Time "mo:base/Time";
import Text "mo:base/Text";
import Trie "mo:base/Trie";
import TrieMap "mo:base/TrieMap";

import PictureRepositories "./PictureRepositories";
import Domain "./Domain";
import Utils "./Utils";

actor PhotoActor {

    type PictureId = Domain.PictureId;
    type Picture = Domain.Picture;
    type PictureProfile = Domain.PictureProfile;

    type UserPrincipal = Domain.UserPrincipal;

    type Error = Domain.PicError;

    type Result<V, E> = Result.Result<V, E>;

    public type PictureStore = PictureRepositories.PictureStore;
    
    // ID生成器
    stable var idGenerator = 10001;

    stable var picsStore: PictureStore = Trie.empty<PictureId, PictureProfile>();

    let pictureRepository : PictureRepositories.PictureRepository = PictureRepositories.PictureRepository();

    /// -------------- query ------------- ///

    /// 查询图片总数
    public query func getTotalPics() : async Result<Nat, Error> {
        #ok(Trie.size<PictureId, PictureProfile>(picsStore))
    };

    /// 获取图片() 公共的获取图片的接口
    /// Args:
    ///     |picId|  需要获取图片的id
    /// Returns:
    ///     如果对应id的图片存在，返回对应的图片信息，否则返回null
    public query func getPictureProfile(picId: PictureId): async Result<?PictureProfile, Error> {
        #ok(pictureRepository.getPictureProfile(picsStore, picId))
    };

    public query func getPicture(picId: PictureId): async Result<?Picture, Error> {
        #ok(pictureRepository.getPicture(picsStore, picId))
    };

    public query func getPictures(picIds: [PictureId]): async Result<[(PictureId, Picture)], Error> {

        #ok(pictureRepository.getPics(picsStore, picIds))
    };

    /// -------------- Update --------------------- ///

    /// savePic() 公共的上传图片接口 需要和前端确认数据格式,图片状态默认是enable
    /// Args:
    ///     |pic|   上传时需要保存的图片内容
    ///     |owner| 图片的所有者
    ///     |picName|   图片的名称
    ///     |description|   对图片的描述
    /// Returns:
    ///     处理成功返回#ok(图片id),,否则返回相应错误
    public shared(msg) func savePic(pic: Picture, picName: Text, description: Text, owner: UserPrincipal) : async Result<PictureId, Error> {
        let picId = getIdAndIncrementOne();

        let current_time = Time.now();
        let createdBy = Principal.toText(msg.caller);
        let picInfo: PictureProfile = {
            id = picId;
            pic = pic;
            picName = picName;
            description = description;
            owner = owner;
            status = #enable;
            createdBy = createdBy;
            createdAt = current_time;
        };

        let picOpt = pictureRepository.getPictureProfile(picsStore, picId);

        switch (picOpt) {
            case (?_) #err(#picAlreadyExisted);
            case null {
                let (newPics, _) = pictureRepository.updatePic(picsStore, picInfo);
                picsStore := newPics;

                #ok(picId)
            };
        }
        
    };

    

    /// 获取当前的id，并对id+1,这是有size effects的操作
    func getIdAndIncrementOne() : Nat {
        let id = idGenerator;
        idGenerator += 1;
        id
    };

    private func extractQueryString(str : Text) : Text {
        let querystring = Text.split(str, #char '?');
        ignore querystring.next(); 
        Utils.getOrEmptyText(querystring.next()) 
    };

    /// 查询图片的http接口,通过url中的querystring方式,picId的值为要指定查询的图片id,http method为GET
    /// e.g. http://localhost:8000?canisterId={canisterId}&picId=10001 或
    /// e.g. https://{canisterId}.raw.ic0.app/?picId=10001
    /// 响应头返回图片格式:Content-Type: image/png(或上传文件时指定的图片格式)
    /// 响应头默认缓存设置: Cache-Control: max-age=86400 (1天)
    /// 响应体是图片对应的的Blob
    public query func http_request(req: HttpRequest) : async HttpResponse {
        let querystring = extractQueryString(req.url);
        Debug.print("querystring is : " # querystring);

        var picIdStr = "";
        let contents = Text.split(querystring, #char '&');
        
        for (c in contents) {
            let cc = Text.split(c, #char '=');
            if (cc.next() == ?"picId") {
                picIdStr := Utils.getOrEmptyText(cc.next());
            };
        };

        let picIdOpt = Utils.textToNat(picIdStr);

        var body: Blob = Text.encodeUtf8("");
        var statusCodeOk : Nat16 = 200; // 200表示ok
        var maxAgeSeconds = "864000"; //表示过期为一天
        var fileType = "image/png";
        switch (picIdOpt) {
            case (?id) { 
                let picOpt = pictureRepository.getPicture(picsStore, id);
                switch (picOpt) {
                    case (?pic) {
                        body := pic.content;
                        fileType := pic.fileType;
                    };
                    case (null) { 
                        statusCodeOk := 404; // 404表示没找到notFound
                        maxAgeSeconds := "60"   // 没找到缓存调协1分钟
                    };
                }
             };
            case (null) { 
                statusCodeOk := 400;     // 400表示请求有问题,badRequest
                maxAgeSeconds := "10"    // 请求有问题设置缓存10秒
            };
        };

        // 响应缓存设置
        let headers = [("Cache-Control", "max-age=" # maxAgeSeconds), ("Content-Type", fileType)];

        return {
            body = body;
            headers = headers;
            status_code = statusCodeOk;
            streaming_strategy = null;
        };
    };

    /// For http_request
    public type HttpRequest = {
        body: Blob;
        headers: [HeaderField];
        method: Text;
        url: Text;
    };

    public type StreamingCallbackToken =  {
        content_encoding: Text;
        index: Nat;
        key: Text;
        sha256: ?Blob;
    };
    public type StreamingCallbackHttpResponse = {
        body: Blob;
        token: ?StreamingCallbackToken;
    };
    public type ChunkId = Nat;

    public type Path = Text;
    public type Key = Text;

    public type HttpResponse = {
        body: Blob;
        headers: [HeaderField];
        status_code: Nat16;
        streaming_strategy: ?StreamingStrategy;
    };

    public type StreamingStrategy = {
        #Callback: {
            callback: query (StreamingCallbackToken) ->
            async (StreamingCallbackHttpResponse);
            token: StreamingCallbackToken;
        };
    };

    public type HeaderField = (Text, Text);
};
