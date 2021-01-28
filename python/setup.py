from setuptools import setup
from setuptools import find_packages

setup(
    name="mfcc-benchmark",
    version="0.1.0",
    description="mfcc",
    install_requires=[
        "numpy",
        "scipy",
        "librosa",
        "pytest-benchmark"
    ],
    packages=find_packages()
)