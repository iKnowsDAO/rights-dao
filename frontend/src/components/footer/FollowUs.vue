<template>
    <div class="follow-us-container">
        <a class="icon-link" target="blank" href="https://t.me/">
            <i class="iconfont icon-telegram"></i>
        </a>
        <a class="icon-link" target="blank" href="https://github.com/dfinity">
            <i class="iconfont icon-github"></i>
        </a>
        <a class="icon-link" target="blank" href="https://twitter.com/">
            <i class="iconfont icon-twitter"></i>
        </a>
        <a class="icon-link" target="blank" href="https://medium.com/">
            <i class="iconfont icon-medium"></i>
        </a>
    </div>
</template>
<script lang="ts" setup></script>
<style lang="scss">
.follow-us-container {
    background-color: black;
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    .icon-link {
        margin-left: 50px;
        margin-right: 50px;
        text-decoration: none;
        color: white;
        > img {
            width: 44px;
            height: 44px;
            cursor: pointer;
        }
        > i {
            font-size: 44px;
            &:hover {
                color: #0fb8e0;
            }
        }
    }
}
@media screen and (max-width: 560px) {
    .follow-us-container {
        display: flex;
        flex-direction: row;
        justify-content: space-around;
        width: 100%;
        .icon-link {
            margin-left: 0px;
            margin-right: 0px;
        }
    }
}
</style>
