
import Array "mo:base/Array";
import Nat "mo:base/Nat";
import Option "mo:base/Option";
import Text "mo:base/Text";
import Trie "mo:base/Trie";

import Domain "./Domain";

module {
    public type PictureId = Domain.PictureId;
    public type Picture = Domain.Picture;
    public type PictureProfile = Domain.PictureProfile;
    public type PictureStore = Domain.PictureStore;

    public type UserPrincipal = Domain.UserPrincipal;
    
    public class PictureRepository() {
        
        public func getPictureProfile(pics: PictureStore, picId: PictureId) : ?PictureProfile {
            Trie.find<PictureId, PictureProfile>(pics, Domain.keyOfPicture(picId), Nat.equal)     
        };

        public func getPicture(pics: PictureStore, picId: PictureId) : ?Picture {
            Option.map<PictureProfile, Picture>(
                getPictureProfile(pics, picId), 
                func (picInfo) : Picture { picInfo.pic }
            )   
        };

        public func getPics(pics: PictureStore, picIds: [PictureId]) : [(PictureId, Picture)] {
            let containPics = Trie.mapFilter<PictureId, PictureProfile, Picture>(pics, func (pid, picInfo) : ?Picture {
                switch (Array.find<PictureId>(picIds, func (id) : Bool { pid == id })) {
                    case (?_) ?picInfo.pic;
                    case _  null;
                }
            });

            Trie.toArray<PictureId, Picture, (PictureId, Picture)>(containPics, func (pid, pic) : (PictureId, Picture) {
                (pid, pic)
            })    
        };

        public func updatePic(pics: PictureStore, picInfo: PictureProfile) : (PictureStore, ?PictureProfile) {
            Trie.put<PictureId, PictureProfile>(pics, Domain.keyOfPicture(picInfo.id), Nat.equal, picInfo);
        };

    };
    
};