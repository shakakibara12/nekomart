import uvicorn
from fastapi import FastAPI
from pydantic import BaseModel

app = FastAPI(title="NekoMart Catalog Service")


# Data Model for a Japanese Product
class Product(BaseModel):
    id: int
    name: str
    category: str  # e.g., Anime, Food, Beauty
    price: float
    image_url: str


@app.get("/")
def read_root():
    return {"service": "catalog", "status": "healthy"}


if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
