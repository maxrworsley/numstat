from num_calc import get_stats
from fastapi import FastAPI
from pydantic import BaseModel


class StatNumber(BaseModel):
    number: int

app = FastAPI()

@app.post("/stats")
def root(number: StatNumber):
    results = get_stats(number.number)
    return results
