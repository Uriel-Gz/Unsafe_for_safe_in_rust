from abc import ABC, abstractmethod


class IPattern(ABC):
    @abstractmethod
    @staticmethod
    def match(self, string: str) -> bool:
        pass

    @abstractmethod
    @staticmethod   
    def replace(self, string: str) -> str:
        pass
