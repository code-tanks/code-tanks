package tanks;

import java.util.HashMap;
import java.util.HashMap;

import api.BaseTank;

public class MyTank extends BaseTank {
    public MyTank() {
        System.out.println("Started MyTank..");

    }

    @Override
    public void run() {
        // TODO Auto-generated method stub
        // throw new UnsupportedOperationException("Unimplemented method 'run'");
        System.out.println("Run1");

    }

    @Override
    public void onEvent(HashMap<String, Object> event) {
        // TODO Auto-generated method stub
        // throw new UnsupportedOperationException("Unimplemented method 'onEvent'");
        System.out.println("onEvent1");
        System.out.println(event);

    }
}