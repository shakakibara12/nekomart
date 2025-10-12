import uvicorn
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from typing import List

app = FastAPI(title="NekoMart Catalog Service")


# Data Model for a Japanese Product
class Product(BaseModel):
    id: int
    name: str
    category: str  # e.g., Anime, Food, Beauty
    price: float
    image_url: str


# Mock Database
PRODUCTS_DB = [
    Product(
        id=1,
        name="Demon Slayer Figure",
        category="Anime",
        price=49.99,
        image_url="/img/naruto.jpg",
    ),
    Product(
        id=2,
        name="Matcha KitKat",
        category="Food",
        price=5.50,
        image_url="/img/kitkat.jpg",
    ),
    Product(
        id=3,
        name="Super Mario Plush",
        category="Games",
        price=25.00,
        image_url="/img/mario.jpg",
    ),
    Product(
        id=4,
        name="K-beauty Serum",
        category="Beauty",
        price=30.00,
        image_url="/img/serum.jpg",
    ),
]


@app.get("/")
def read_root():
    return {"service": "catalog", "status": "healthy"}


if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
