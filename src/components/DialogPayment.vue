<script setup lang="ts">
import { ref, defineProps, defineEmits, watch } from "vue";

const paymentDialog = ref<HTMLDialogElement | null>(null); // Ref สำหรับ <dialog>
// รับ props จาก parent component
const props = defineProps<{
    isPaymentSuccess: boolean;
}>();

// Watch สำหรับอัปเดต amountPaid และเปิด/ปิด Dialog เมื่อ props เปลี่ยน
watch(
    () => props.isPaymentSuccess,
    (newValue) => {
        if (newValue) {
            paymentDialog.value?.showModal(); // เปิด Dialog
        } else {
            paymentDialog.value?.close(); // ปิด Dialog
        }
    },
);

const emit = defineEmits(["close"]);

// ฟังก์ชันปิด Dialog
const closeDialog = () => {
    emit("close");
};
</script>

<template>
    <div>
        <!-- Dialog สำหรับแสดงผลการชำระเงิน -->
        <dialog
            aria-labelledby="modal-title"
            role="dialog"
            aria-modal="true"
            ref="paymentDialog"
            class="w-2/5 rounded-lg bg-white shadow-sm"
        >
            <div class="flex flex-col p-4">
                <h2 class="text-2xl font-bold text-green-600 mb-4 text-center">
                    ชำระเงินสำเร็จ!
                </h2>
                <button
                    @click="closeDialog"
                    class="bg-green-500 text-white px-4 py-2 rounded hover:bg-green-600 transition-colors w-full"
                >
                    ปิด
                </button>
            </div>
        </dialog>
    </div>
</template>
