import Char "mo:base/Char";
import Iter "mo:base/Iter";
import Nat "mo:base/Nat";
import Nat32 "mo:base/Nat32";
import Option "mo:base/Option";
import Text "mo:base/Text";

module {

    /// 把Text转为Nat,如果Text中包含非0-9的数字,返回null
    public func textToNat(t: Text) : ?Nat {
        var size : Nat = t.size();
        if (size > 20) return null;

        let chars = Text.toIter(t);

        let nats = Iter.map<Char, Nat>(chars, func (c) : Nat { Nat32.toNat(Char.toNat32(c))});
        
        var num = 0;
        for (n in nats) {
            if (n < 48 or n > 57) return null;
            let v: Nat = n - 48;
            size -= 1;
            num += Nat.pow(10, size) * v;
        };
        ?num
    };

    public func getOrEmptyText(str: ?Text) : Text {
        Option.get(str, "")
    };
}