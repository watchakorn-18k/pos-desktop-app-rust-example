<script setup>
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
// ข้อมูลสินค้า
const products = ref([]);

// ตะกร้าสินค้า
const cart = ref([]);

// ฟังก์ชันเพิ่มสินค้าในตะกร้า
const addToCart = (product) => {
  const found = cart.value.find((item) => item.name === product.name);
  if (found) {
    found.quantity++;
  } else {
    cart.value.push({ ...product, quantity: 1 });
  }
};

// ฟังก์ชันลบสินค้าจากตะกร้า
const removeFromCart = (item) => {
  const index = cart.value.indexOf(item);
  if (index !== -1) {
    cart.value.splice(index, 1);
  }
};

// คำนวณยอดรวม
const totalAmount = computed(() => {
  return cart.value.reduce(
    (total, item) => total + item.price * item.quantity,
    0
  );
});

// ฟังก์ชันการจ่ายเงิน
const checkout = () => {
  alert(`ยอดรวมทั้งหมด: ฿${totalAmount.value.toFixed(2)}\nการชำระเงินสำเร็จ`);
  cart.value = [];
};

onMounted(async () => {
  products.value = await invoke("get_foods");
  console.log(products.value);
});
</script>

<template>
  <div class="bg-gray-100 min-h-screen flex flex-col justify-between w-screen">
    <header class="bg-blue-600 text-white p-4 shadow-lg">
      <h1 class="text-2xl font-bold">POS System</h1>
    </header>

    <main class="flex-grow p-6">
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <!-- รายการสินค้า -->
        <div class="bg-white p-4 rounded-lg shadow-md">
          <h2 class="text-xl font-semibold mb-4">สินค้า</h2>
          <ul>
            <li
              v-for="(product, index) in products"
              :key="index"
              class="flex justify-between items-center py-2 border-b"
            >
              <span>{{ product.name }}</span>
              <span>฿{{ product.price.toFixed(2) }}</span>
              <button
                class="bg-blue-500 text-white px-3 py-1 rounded"
                @click="addToCart(product)"
              >
                เพิ่ม
              </button>
            </li>
          </ul>
        </div>

        <!-- ตะกร้าสินค้า -->
        <div class="bg-white p-4 rounded-lg shadow-md">
          <h2 class="text-xl font-semibold mb-4">ตะกร้าสินค้า</h2>
          <ul>
            <li
              v-for="(item, index) in cart"
              :key="index"
              class="flex justify-between items-center py-2 border-b"
            >
              <span>{{ item.name }} x{{ item.quantity }}</span>
              <span>฿{{ (item.price * item.quantity).toFixed(2) }}</span>
              <button
                class="bg-red-500 text-white px-3 py-1 rounded"
                @click="removeFromCart(item)"
              >
                ลบ
              </button>
            </li>
          </ul>

          <div class="mt-4 flex justify-between items-center">
            <span class="font-bold">ยอดรวม:</span>
            <span>฿{{ totalAmount.toFixed(2) }}</span>
          </div>

          <button
            class="mt-6 bg-green-500 text-white py-2 px-4 w-full rounded"
            @click="checkout"
          >
            จ่ายเงิน
          </button>
        </div>
      </div>
    </main>

    <footer class="bg-blue-600 text-white p-4 text-center">
      <p>&copy; 2024 ระบบ POS</p>
    </footer>
  </div>
</template>
