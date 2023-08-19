<script setup>
import { ref, onMounted } from 'vue';
import { io } from 'socket.io-client';
    const data = ref("");
    const question = ref("");
    const socket = io("http://23.111.144.10:3000/");
    const loading = ref(false);
    const sendQuestion = () => {
      loading.value = true;
      const config = {
        seed: -1,
        threads: 4,
        n_predict: 200,
        top_k: 40,
        top_p: 0.9,
        temp: 0.8,
        repeat_last_n: 64,
        repeat_penalty: 1.3,
        debug: false,
        models: ["llama.13B"],
        model: "llama.13B",
        prompt: question.value,
        id: "TS-1691809628841-75297",
      };
      data.value="";
      socket.emit("request", config);
    };

    const cancelRequest = () => {
      loading.value = false;
      socket.emit("request", {
        prompt: "/stop",
      });
    };

    onMounted(() => {    
    socket.emit("request", {
      method: "installed",
    });

    socket.on("result", ({ request, response }) => {
      if (request.method === "installed") {
        console.log("installed", response);
      } else {
        if (response === "\n\n<end>") {
          loading.value = false
        } else {
          data.value += response;
        }
      }
    });
    });

</script>

<template>
    <div class="note-tag-container">
      <div class="note-tag-header">
        <p>InfinitiNote AI</p>
      </div>
      <div class="note-tag-chip-container">
        <textarea v-model="question" cols="30" rows="10" placeholder="Please enter your question"></textarea>
      </div>
      <div class="note-action-buttons">
        <button class="note-action-buttons-ok" @click="loading?cancelRequest():sendQuestion()">{{loading?'Cancel':'Send'}}</button>
      </div>
      <div class="note-tag-footer" v-if="data">
        <p>Answer:</p>
        <p style="font-size: 12px;">{{ data }}</p>
      </div>
    </div>
  </template>
  


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
.note-tag-container textarea{
border:1px solid #EDEFF3;
width: 100%;
padding: 10px;
border-radius: 15px;
}
.note-tag-container .note-action-buttons{
margin-top: 10px;
}
.note-tag-container .note-action-buttons-ok{
    width: 100%;
    
}
.chat-bot-answer{
    font-size: 14px;
    font-weight: 400;
    margin-top: 0px;
    margin-bottom: 20px;
}
</style>