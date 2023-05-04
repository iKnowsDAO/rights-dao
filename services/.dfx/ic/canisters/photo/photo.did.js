export const idlFactory = ({ IDL }) => {
    const PictureId = IDL.Nat;
    const Picture = IDL.Record({
        'content' : IDL.Vec(IDL.Nat8),
        'fileType' : IDL.Text,
    });
    const Error = IDL.Variant({
        'picNotFound' : IDL.Null,
        'tooLargeQuantity' : IDL.Null,
        'picAlreadyExisted' : IDL.Null,
    });
    const Result_4 = IDL.Variant({ 'ok' : IDL.Opt(Picture), 'err' : Error });
    const PictureId__1 = IDL.Nat;
    const Picture__1 = IDL.Record({
        'content' : IDL.Vec(IDL.Nat8),
        'fileType' : IDL.Text,
    });
    const PictureStatus = IDL.Variant({
        'pending' : IDL.Null,
        'disable' : IDL.Null,
        'enable' : IDL.Null,
    });
    const UserPrincipal__1 = IDL.Text;
    const Timestamp = IDL.Int;
    const PictureProfile = IDL.Record({
        'id' : PictureId__1,
        'pic' : Picture__1,
        'status' : PictureStatus,
        'owner' : UserPrincipal__1,
        'createdAt' : Timestamp,
        'createdBy' : UserPrincipal__1,
        'description' : IDL.Text,
        'picName' : IDL.Text,
    });
    const Result_3 = IDL.Variant({
        'ok' : IDL.Opt(PictureProfile),
        'err' : Error,
    });
    const Result_2 = IDL.Variant({
        'ok' : IDL.Vec(IDL.Tuple(PictureId, Picture)),
        'err' : Error,
    });
    const Result_1 = IDL.Variant({ 'ok' : IDL.Nat, 'err' : Error });
    const HeaderField = IDL.Tuple(IDL.Text, IDL.Text);
    const HttpRequest = IDL.Record({
        'url' : IDL.Text,
        'method' : IDL.Text,
        'body' : IDL.Vec(IDL.Nat8),
        'headers' : IDL.Vec(HeaderField),
    });
    const StreamingCallbackToken = IDL.Record({
        'key' : IDL.Text,
        'sha256' : IDL.Opt(IDL.Vec(IDL.Nat8)),
        'index' : IDL.Nat,
        'content_encoding' : IDL.Text,
    });
    const StreamingCallbackHttpResponse = IDL.Record({
        'token' : IDL.Opt(StreamingCallbackToken),
        'body' : IDL.Vec(IDL.Nat8),
    });
    const StreamingStrategy = IDL.Variant({
        'Callback' : IDL.Record({
            'token' : StreamingCallbackToken,
            'callback' : IDL.Func(
                [StreamingCallbackToken],
                [StreamingCallbackHttpResponse],
                ['query'],
            ),
        }),
    });
    const HttpResponse = IDL.Record({
        'body' : IDL.Vec(IDL.Nat8),
        'headers' : IDL.Vec(HeaderField),
        'streaming_strategy' : IDL.Opt(StreamingStrategy),
        'status_code' : IDL.Nat16,
    });
    const UserPrincipal = IDL.Text;
    const Result = IDL.Variant({ 'ok' : PictureId, 'err' : Error });
    return IDL.Service({
        'getPicture' : IDL.Func([PictureId], [Result_4], ['query']),
        'getPictureProfile' : IDL.Func([PictureId], [Result_3], ['query']),
        'getPictures' : IDL.Func([IDL.Vec(PictureId)], [Result_2], ['query']),
        'getTotalPics' : IDL.Func([], [Result_1], ['query']),
        'http_request' : IDL.Func([HttpRequest], [HttpResponse], ['query']),
        'savePic' : IDL.Func(
            [Picture, IDL.Text, IDL.Text, UserPrincipal],
            [Result],
            [],
        ),
    });
};
export const init = ({ IDL }) => { return []; };
