import os
import pytest
import csv
from main import process_csv


@pytest.fixture
def input_file(tmpdir):
    file_path = os.path.join(tmpdir, "test_input.csv")
    with open(file_path, "w", newline="") as csvfile:
        csvwriter = csv.writer(csvfile)
        csvwriter.writerow(["header1", "header2"])
        csvwriter.writerow(["row1", "10"])
        csvwriter.writerow(["row2", "20"])
    return file_path


@pytest.fixture
def output_file(tmpdir):
    return os.path.join(tmpdir, "output.txt")


def test_process_csv(input_file, output_file):
    process_csv(input_file, output_file)

    with open(output_file, "r") as f:
        content = f.read()
        assert content == "Average: 15.00"
