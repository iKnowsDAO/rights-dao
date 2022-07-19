<template>
    <div class="footer-container">
        <FollowUs class="follow-us" />
        <Copyright class="copyright" />
    </div>
</template>
<script lang="ts" setup>
import FollowUs from './FollowUs.vue';
import Copyright from './Copyright.vue';
</script>
<style lang="scss">
.footer-container {
    background-color: black;
    padding-top: 100px;
    padding-bottom: 50px;
    .additions {
        margin-top: 100px;
    }
    .copyright {
        margin-top: 100px !important;
    }
}
</style>
