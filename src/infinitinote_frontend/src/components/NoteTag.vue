<template>
    <div class="note-tag-container">
        <div class="note-tag-header">
            <p>Notebook Tags</p>
            <span>
                <img src="/ui/add.svg" alt="" srcset="">

            </span>
        </div>
        <div class="note-tag-chip-container">
            <div class="note-tag-chip">
                <span>
                    swift
                </span>
                <span>
                    <img src="/ui/close.svg" alt="" srcset="">
                </span>
            </div>
            <div class="note-tag-chip">
                <span>
                    swift
                </span>
                <span>
                    <img src="/ui/close.svg" alt="" srcset="">
                </span>
            </div>
            <div class="note-tag-chip">
                <span>
                    swift
                </span>
                <span>
                    <img src="/ui/close.svg" alt="" srcset="">
                </span>
            </div>
            <div class="note-tag-chip">
                <span>
                    swift
                </span>
                <span>
                    <img src="/ui/close.svg" alt="" srcset="">
                </span>
            </div>
            <div class="note-tag-chip">
                <span>
                    swift
                </span>
                <span>
                    <img src="/ui/close.svg" alt="" srcset="">
                </span>
            </div>

        </div>
    </div>
</template>

<script setup>
</script>

<style>
.note-tag-container {
    width: 276px;
    min-height: 254px;
    border-radius: 25px;
    background: #FFF;
    padding: 29px 32px;
}

.note-tag-header {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
}

.note-tag-header p {
    color: #2B3249;
    font-size: 16px;
    font-weight: 600;
}

.note-tag-header span {
    display: flex;
    width: 22px;
    height: 22px;
    padding: 14px;
    justify-content: center;
    align-items: center;
    border-radius: 50%;
    line-height: 0;
    background: rgba(34, 97, 211, 0.10);
}

.note-tag-chip-container {
    display: flex;
    flex-direction: row;
    /* or column, depending on your layout */
    flex-wrap: wrap;
    margin-top: 15px;
    align-items: flex-start;
    /* aligns items to the start of the container */

}

.note-tag-chip {
    display: inline-flex;
    padding: 9px 8px 9px 12px;
    justify-content: center;
    align-items: center;
    gap: 7px;
    margin: 10px 5px;
    border-radius: 10px;
    background: rgba(34, 97, 211, 0.10);
}

.note-tag-chip span:nth-child(1) {
    font-size: 16px;
    font-weight: 600;
}
</style>