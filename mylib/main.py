import csv
import argparse


def process_csv(input_file, output_file):
    total = 0
    count = 0

    with open(input_file, "r") as file:
        reader = csv.reader(file)
        next(reader)  # Skip header

        for row in reader:
            try:
                value = int(row[1])
                total += value
                count += 1
            except ValueError:
                pass  # Ignore non-integer values

    average = total / count if count > 0 else 0

    with open(output_file, "w") as output:
        output.write(f"Average: {average:.2f}")


def main():
    parser = argparse.ArgumentParser(
        description="Process CSV file and calculate average."
    )
    parser.add_argument("input_file", type=str, help="Input CSV file path")
    parser.add_argument("output_file", type=str, help="Output text file path")

    args = parser.parse_args()

    process_csv(args.input_file, args.output_file)


if __name__ == "__main__":
    main()