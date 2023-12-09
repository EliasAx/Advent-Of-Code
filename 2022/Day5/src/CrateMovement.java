public class CrateMovement {
    int from;
    int to;
    int amount;

    public int getFrom() {
        return from;
    }

    public int getTo() {
        return to;
    }

    public int getAmount() {
        return amount;
    }

    public void setFrom(int from) {
        this.from = from;
    }

    public void setTo(int to) {
        this.to = to;
    }

    public void setAmount(int amount) {
        this.amount = amount;
    }

    public String toString() {
        return "From: " + from + " To: " + to + " Amount: " + amount;
    }
}
