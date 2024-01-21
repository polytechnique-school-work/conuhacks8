<template>
    <div class="container">
        <div class="category">
        <p>Edit Number of Served Cars</p>
            <vehicle-type-card type="Compact Cars" :amount=10 :isTruck=false ></vehicle-type-card>
            <vehicle-type-card type="Medium Size Cars" :amount=10 :isTruck=false ></vehicle-type-card>
            <vehicle-type-card type="Full Size Cars" :amount=10 :isTruck=false ></vehicle-type-card>
            <vehicle-type-card type="Class 1 Truck" :amount=10 :isTruck=true ></vehicle-type-card>
            <vehicle-type-card type="Class 2 Truck" :amount=10 :isTruck=true ></vehicle-type-card>
        </div>
        <div class="category">
        <p>Edit Number of Declined Cars</p>
            <vehicle-type-card type="Compact Cars" :amount=10 :isTruck=false ></vehicle-type-card>
            <vehicle-type-card type="Medium Size Cars" :amount=10 :isTruck=false ></vehicle-type-card>
            <vehicle-type-card type="Full Size Cars" :amount=10 :isTruck=false ></vehicle-type-card>
            <vehicle-type-card type="Class 1 Truck" :amount=10 :isTruck=true ></vehicle-type-card>
            <vehicle-type-card type="Class 2 Truck" :amount=10 :isTruck=true ></vehicle-type-card>
        </div>
    </div>
</template>

<style scoped lang="scss">
@import "../assets/style/constants.scss";
.container{
    height: 100%;
    width: 100%;
    margin: 10px;
    font-family: "Noto Sans", sans-serif;
    display: flex;
    flex-direction: row;
}

.category{
    padding: 15px;
    background-color: #FFFFFF;
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    height: 100;
    margin: 10px
}

</style>