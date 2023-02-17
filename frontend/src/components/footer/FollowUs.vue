<template>
    <div class="follow-us-container">
        <b>©</b>
        <b class="copyright">&nbsp;iKnows</b>
        <!--<a class="icon-link" target="blank" href="https://t.me/">-->
            <!--<i class="iconfont icon-telegram"></i>-->
        <!--</a>-->
        <!--<a class="icon-link" target="blank" href="https://github.com/dfinity">-->
            <!--<i class="iconfont icon-github"></i>-->
        <!--</a>-->
        <a class="icon-link" target="blank" href="https://twitter.com/iKnowsDAO">
            <i class="iconfont icon-twitter"></i>
        </a>
        <a class="icon-link" target="blank" href="https://discord.gg/3ZUUaaWrSE">
            <i class="iconfont icon-discord"></i>
        </a>
    </div>
</template>
<script lang="ts" setup></script>
<style lang="scss">
.follow-us-container {
    display: flex;
    flex-direction: row;
    /*justify-content: center;*/
    align-items: center;
    .copyright{
        font-size: 15px;
        margin-right: 10px;
    }
    .icon-link {
        margin-right: 10px;
        text-decoration: none;
        color: rgb(113,113,122);
        > img {
            width: 22px;
            height: 22px;
            cursor: pointer;
        }
        > i {
            font-size: 22px;
            &:hover {
                filter: drop-shadow(0 0 0 black);
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
