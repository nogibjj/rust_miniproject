import csv

def main():
    total = 0
    count = 0

    with open('input.csv', 'r') as file:
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

    with open('output.txt', 'w') as output:
        output.write(f'Average: {average:.2f}')

if __name__ == "__main__":
    main()