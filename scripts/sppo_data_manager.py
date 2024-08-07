import fire
import pandas as pd
import json
import glob

class SolarPanelPlacementOptimizerDataManager:
    def __init__(self):
        self.df = None
        self._config_file = "etc/sppo-data-manager/config.json"
        
        with open(self._config_file, 'r') as f:
            self._config = json.load(f)
        print("Configuration Settings:")
        print(json.dumps(self._config, indent=2))

    def create(self):
        DATA_FILEs = glob.glob(self._config["data_file_dir"]+"/*")
        for FILE in DATA_FILEs:
            print(FILE)
    
if __name__ == '__main__':
    fire.Fire(SolarPanelPlacementOptimizerDataManager)
