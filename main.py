import random
import aiohttp
import asyncio
from abc import ABC, abstractmethod
import nest_asyncio

nest_asyncio.apply()


class Currency(ABC):
    """ Абстрактный класс валюты. """

    def __init__(self, amount: float):
        """ Инициализация валюты с суммой. """
        self.amount = amount

    @abstractmethod
    def to_rubles(self) -> float:
        """ Абстрактный метод для перевода в рубли. """
        pass

    @abstractmethod
    def display(self) -> str:
        """ Абстрактный метод для отображения суммы. """
        pass

    @abstractmethod
    async def get_exchange_rate(self):
        """ Абстрактный метод для получения актуального курса валюты. """
        pass


class Dollar(Currency):
    async def get_exchange_rate(self):
        """ Получает актуальный курс доллара к рублю. """
        async with aiohttp.ClientSession() as session:
            async with session.get("https://api.exchangerate-api.com/v4/latest/USD") as response:
                data = await response.json()
                return float(data['rates']['RUB'])

    async def to_rubles(self) -> str:
        rate = await self.get_exchange_rate()
        return f"₽{self.amount * rate:.2f}"

    def display(self) -> str:
        """ Отображает сумму в долларах. """
        return f"${self.amount:.2f}"


class Euro(Currency):
    async def get_exchange_rate(self):
        """ Получает актуальный курс евро к рублю. """
        async with aiohttp.ClientSession() as session:
            async with session.get("https://api.exchangerate-api.com/v4/latest/EUR") as response:
                data = await response.json()
                return float(data['rates']['RUB'])

    async def to_rubles(self) -> str:
        rate = await self.get_exchange_rate()
        return f"₽{self.amount * rate:.2f}"

    def display(self) -> str:
        """ Отображает сумму в евро. """
        return f"€{self.amount:.2f}"


async def main():
    dollar_amount = random.uniform(1, 100)
    euro_amount = random.uniform(1, 100)

    dollar_instance = Dollar(dollar_amount)
    euro_instance = Euro(euro_amount)

    # Асинхронный вызов методов
    output_results = [
        (dollar_instance.display(), await dollar_instance.to_rubles()),
        (euro_instance.display(), await euro_instance.to_rubles())
    ]

    print(output_results)


# Запуск асинхронного main
asyncio.run(main())
