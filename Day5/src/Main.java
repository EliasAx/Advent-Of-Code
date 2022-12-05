import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.Scanner;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Main {
    public static void main(String[] args) {
        Main main = new Main();
        try {
            File input = new File("Input.txt");
            Scanner inputReader = new Scanner(input);
            Scanner inputReader2 = new Scanner(input);

            HashMap<Integer, ArrayList<String>> crateStacks = main.getCrateStacks(inputReader);
            HashMap<Integer, ArrayList<String>> crateStacks2 = main.getCrateStacks(inputReader2);

            ArrayList<CrateMovement> crateMovements = main.getCrateMovements(inputReader);

            main.part1(crateStacks, crateMovements);
            main.part2(crateStacks2, crateMovements);

            inputReader.close();
            inputReader2.close();
        } catch (FileNotFoundException e) {
            System.out.println("An error occurred.");
            e.printStackTrace();
        }
    }

    void part1(HashMap<Integer, ArrayList<String>> crateStacks, ArrayList<CrateMovement> crateMovements) {
        for (CrateMovement movement : crateMovements) {
            for (int i = 0; i < movement.amount; i++) {
                crateStacks.get(movement.to).add(crateStacks.get(movement.from).remove(crateStacks.get(movement.from).size() - 1));
            }
        }

        String answer = "";
        for (ArrayList<String> crates : crateStacks.values()) {
            answer += crates.get(crates.size()-1);
        }

        System.out.println("Part 1: " + answer);
    }

    void part2(HashMap<Integer, ArrayList<String>> crateStacks, ArrayList<CrateMovement> crateMovements) {
        for (CrateMovement movement : crateMovements) {
            int fromLastIndex = crateStacks.get(movement.from).size();
            crateStacks.get(movement.to).addAll(crateStacks.get(movement.from).subList(fromLastIndex - movement.amount, fromLastIndex));
            crateStacks.put(movement.from, new ArrayList<>(crateStacks.get(movement.from).subList(0, fromLastIndex - movement.amount)));
        }

        String answer = "";
        for (ArrayList<String> crates : crateStacks.values()) {
            answer += crates.get(crates.size()-1);
        }

        System.out.println("Part 2: " + answer);
    }

    HashMap<Integer, ArrayList<String>> getCrateStacks(Scanner reader) {
        HashMap<Integer, ArrayList<String>> crateStacks = new HashMap<>();
        while (reader.hasNextLine()) {
            String line = reader.nextLine();
            if (line.contains("1")) { // Don't use the stack number input
                continue;
            }
            if (line.trim().length() == 0) { // Stop reading when we've read all crate stacks
                break;
            }
            char[] characters = line.toCharArray();
            for(int i = 0; i < characters.length; i++) {
                if (!Character.isWhitespace(characters[i])) {
                    int stack;
                    if (i == 0) {
                        stack = 1;
                    } else {
                        stack = (i / 4) + 1; //Four spaces between each row
                    }
                    if (crateStacks.containsKey(stack)) {
                        crateStacks.get(stack).add(0, String.valueOf(characters[i + 1]));
                    } else {
                        crateStacks.put(stack, new ArrayList<>());
                        crateStacks.get(stack).add(0, String.valueOf(characters[i + 1]));
                    }

                    i += 2;
                }
            }
        }
        return crateStacks;
    }

    ArrayList<CrateMovement> getCrateMovements(Scanner reader) {
        ArrayList<CrateMovement> movements = new ArrayList<>();

        while (reader.hasNextLine()) {
            String line = reader.nextLine();

            Pattern integerPattern = Pattern.compile("-?\\d+");
            Matcher matcher = integerPattern.matcher(line);

            CrateMovement movement = new CrateMovement();
            int index = 0;
            while (matcher.find()) {
                if(index == 0)
                    movement.setAmount(Integer.parseInt(matcher.group()));
                else if(index==1)
                    movement.setFrom(Integer.parseInt(matcher.group()));
                else
                    movement.setTo(Integer.parseInt(matcher.group()));
                index++;
            }
            movements.add(movement);
        }
        return movements;
    }

}
