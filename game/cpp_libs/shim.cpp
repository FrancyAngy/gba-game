#include <LinkCard.hpp>
#include <new>

static LinkCard *card = nullptr;
alignas(LinkCard) static unsigned char card_buf[sizeof(LinkCard)];

extern "C" {

void shim_card_init(void) {
  if (!card)
    card = new (card_buf) LinkCard();
}

int shim_card_get_device(bool (*cancel)(void)) {
  return (int)card->getConnectedDevice(cancel);
}

int shim_card_send_loader(const unsigned char *loader, unsigned int size,
                          bool (*cancel)(void)) {
  return (int)card->sendLoader(loader, size, cancel);
}

int shim_card_receive(unsigned char *buf, bool (*cancel)(void)) {
  return (int)card->receiveCard(buf, cancel);
}
}
