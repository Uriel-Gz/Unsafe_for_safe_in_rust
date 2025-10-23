from abc import ABC, abstractmethod


class IPattern(ABC):
    @staticmethod
    @abstractmethod
    def match(self, string: str) -> bool:
        pass

    @staticmethod   
    @abstractmethod
    def replace(self, string: str) -> str:
        pass
