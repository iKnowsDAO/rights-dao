<template>
    <div class="copyright text-center mt-3">
        <p class="mb-0">
            {{
                `Copyright © ${
                    2021 + (new Date().getFullYear() === 2021 ? '' : '-' + new Date().getFullYear())
                } iKnows. All rights reserved`
            }}
        </p>
    </div>
</template>
<script lang="ts" setup></script>
<style lang="scss">
.copyright {
    /*background-color: black;*/
    > p {
        font-size: 12px;
        margin: 0 auto;
        color: #676767;
        user-select: none;
    }
}
</style>
