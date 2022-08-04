from setuptools import setup, find_packages

requirements = [
    'bleak',
    'paho-mqtt'
]

setup(
    name='ccmqtt',
    version='0.1',
    description='MQTT support package for BLE Tenka Circuit Cubes',
    url='http://github.com/dsobotta/mqtt-circuit-cubes',
    author='Dusten Sobotta',
    author_email='dusten@hey.com',
    keywords=['tenka', 'circuitcubes', 'circuit', 'cubes', 'ble', 'gatt', 'mqtt'],
    packages=find_packages('src'),
    package_dir={'': 'src'},
    install_requires=requirements,
    zip_safe=False
)
