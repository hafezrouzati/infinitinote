<template>
    <div class="signup-container">
        <div class="left">
            <img src="/ui/login-image.png" alt="">
            <div class="banner-text">
                <span class="first">Powered</span>
                <span class="second">by Internet Computer</span>
                <span class="third">
                    Infiniti Note: Where the Unbounded Universe of Blockchain and AI Enhances Your Notes with Endless Insights.
                </span>
            </div>
        </div>
        <div class="right">
            <div class="registration-container">
                <div class="registration-title">Sign Up</div>
                <img src="/ui/icp_login.png" class="image_button" @click="authenticate_icp()">
                <!-- <div class="registration-field">
                    <div class="registration-input"></div>
                    <span class="registration-label">Your email</span>
                </div>
                <div class="login-options">
                    <div class="login-option">
                        <div class="login-icon"></div>
                        <span class="login-label">Login with Internet Identity</span>
                        <div class="login-icon-img"></div>
                    </div>
                </div>
                <div class="login-link">
                    <span class="login-question">Already have an account?</span>
                    <span class="login-btn">Login</span>
                </div>
                <span class="signup-btn">Sign Up</span>
                <div class="social-login">
                    <span class="social-label">or login with</span>
                    <div class="social-icons">
                        <div class="social-icon-twitter"></div>
                        <div class="social-icon-google"></div>
                        <div class="social-icon-apple"></div>
                        <div class="social-icon-metamask"></div>
                    </div>
                </div> -->
            </div>


        </div>
    </div>
</template>
<style>
.registration-container {
    margin-top: -26px;
    border-width: 0px, 0px, 0px, 0px;
    border-style: solid;
    border-color: #C2C2C2;
    background: #FFFFFF;
    padding: 20px 40px;
    border-radius: 25px;
    margin-left: 20px;
    margin-right: 20px;
}

.registration-title {
    font-family: Montserrat;
    font-size: 30px;
    font-weight: 600;
    line-height: 37px;
    letter-spacing: -0.03em;
    text-align: center;
    color: #2B3249;
    margin-top: 20px;
    margin-bottom: 50px;

}

.image_button {
    margin-bottom: 20px;
}

.signup-container {
    display: flex;

}

.signup-container .banner-text {
    margin-left: 100px;
    margin-top: -50px;
    display: flex;
    flex-direction: column;
}

.signup-container .banner-text .first {
    font-family: Montserrat;
    font-size: 60px;
    font-weight: 600;
    line-height: 67px;
    letter-spacing: -0.03em;
    text-align: left;
    color: #2B3249;


}

.signup-container .banner-text .second {
    font-family: Montserrat;
    font-size: 60px;
    font-weight: 600;
    line-height: 67px;
    letter-spacing: -0.03em;
    text-align: left;
    color: #2B3249;
    opacity: 50%;

}

.signup-container .banner-text .third {
    font-family: Montserrat;
    font-size: 16px;
    font-weight: 500;
    line-height: 26px;
    letter-spacing: -0.03em;
    text-align: left;
    color: #2B3249;
    margin-top: 30px;
}

.signup-container .left {
    flex: 1;
}

.signup-container .left img {
    width: 1034px;
    height: 654px;
    max-width: unset;
    max-height: unset;
    border-radius: 30px;
    margin-top: -111px;

}

.signup-container .right {
    flex: 1;
}
</style>
<script setup>
import { AuthClient } from '@dfinity/auth-client';
import { Actor, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { infinitinote_backend, canisterId, createActor } from '../../../declarations/infinitinote_backend';
import { inject, onMounted, ref } from 'vue';
import { useRouter, useRoute } from 'vue-router';
var userAuthenticated = inject('userAuthenticated');
var userPrincipal = inject('userPrincipal');
var isLoading = inject('isLoading');
var backend = inject('backend');

const router = useRouter()
const route = useRoute();

var authClient = null;

async function onAuthenticated_icp(authClient) {
    userAuthenticated.value = true;

    let userIdentity = await authClient.getIdentity();
    userPrincipal.value = userIdentity.getPrincipal().toString();

    var actorOptions = {
        agentOptions: {
            identity: userIdentity,
        }
    };

    var authenticated_backend = createActor(canisterId, actorOptions);

    backend.value = authenticated_backend;

    router.push('/home');
}

async function authenticate_icp() {
    authClient = await AuthClient.create();

    await authClient.login({
        identityProvider: process.env.II_URL,
        onSuccess: async () => {
            onAuthenticated_icp(authClient);
        },
    });
}

onMounted(async () => {
    isLoading.value = false;
    if (userAuthenticated.value == true) {
        router.push('/home');
    }
});

</script>

<style>
.collaboration-cursor__label {
    font-size: 18px !important;
    width: 133px;
    height: 24px;
    border-radius: 30px;
    padding: 10px;
}
</style>