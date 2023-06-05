<template>
    <div class="footer-container">
        <FollowUs class="follow-us" />
        <!--<Copyright class="copyright" />-->
        <div class="ic-powered">
            <img src="@/assets/images/ic-powered-by-crypto.svg">
        </div>
    </div>
</template>
<script lang="ts" setup>
import FollowUs from './FollowUs.vue';
import Copyright from './Copyright.vue';
</script>
<style lang="scss">
    .footer-container {
        padding-top: 10px;
        /*padding-bottom: 50px;*/
        .ic-powered {
            img {
                width: 150px;
            }
        }
    }
</style>
