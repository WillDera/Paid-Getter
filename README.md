# Paid Greeter
This is a modification of the greeter project showcased in the [start building dApps on ink!](https://www.eventbrite.co.uk/e/827155583617?aff=oddtdtcreator) workshop. 

## Modification
- Callers have to pay a small amount to write a message to the contract
- Contract keeps track of how many messages have been set in the contract or basically how many times the set message endpoint has been called.
- At every 5th interval of messages, the caller wins the total amount of tokens that the contract received from previous callers.
- The next caller or the current caller then has to unlock the contract to set a new message, or else the contract remains locked.

## Demo Video
You can find my [DEMO Video Here](https://www.veed.io/view/43212011-22ac-46bc-82a7-36dbf389d541?panel=share)

## Images
> Take note of the Fetched Message and Fetched Lock State in each image
- Caller paying to set message
![Screenshot from 2024-03-09 17-44-05](https://github.com/WillDera/inkathon-project/assets/30760648/7ecc692b-5e14-4103-bf8f-d0a2d2d8be83)
- Contract picking a winner
![Screenshot from 2024-03-09 17-45-54](https://github.com/WillDera/inkathon-project/assets/30760648/dbd0a4d5-85df-4052-a14d-fecef941482c)
- Caller unlocking contract before new message can be set
![Screenshot from 2024-03-09 17-45-54](https://github.com/WillDera/inkathon-project/assets/30760648/871a93d1-eb84-47fb-af04-73409485bf0e)
![Screenshot from 2024-03-09 17-47-24](https://github.com/WillDera/inkathon-project/assets/30760648/29f83e9d-bac4-4f35-ba4f-257de83de511)
