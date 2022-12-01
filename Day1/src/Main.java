import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Scanner;

public class Main {

    private static ArrayList<Integer> elfCalories = new ArrayList<>();

    public static void main(String[] args) {

        int currentElf = 0;
        elfCalories.add(0);

        try {
            File input = new File("Input.txt");
            Scanner inputReader = new Scanner(input);
            while (inputReader.hasNextLine()) {
                String data = inputReader.nextLine();
                if(data.trim().length() != 0)
                {
                    int calories = Integer.parseInt(data);
                    elfCalories.set(currentElf, elfCalories.get(currentElf)+calories);
                } else {
                    elfCalories.add(0);
                    currentElf++;
                }
            }
            inputReader.close();
        } catch (FileNotFoundException e) {
            System.out.println("An error occurred.");
            e.printStackTrace();
        }

        System.out.println(elfCalories.size());
        System.out.println(elfCalories);
        System.out.println(Collections.max(elfCalories));
    }
}
