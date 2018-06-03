import { Component } from "inferno";

import { CourierState } from "../utils/data_parsers";
import {
  AbbreviatedSingleStat,
  PercentageSingleStat,
  SingleStat,
  SizeSingleStat,
} from "./single_stat";

interface Props {
  courierState: CourierState;
}

export function AllSingleStats(props: Props) {
  const s = props.courierState;
  return (
    <section class="section">
      <div class="container">
        {/* Current Stats */}
        <div class="level">
          <h1 class="is-size-5">Current</h1>
          <AbbreviatedSingleStat title="Topics" value={s.numTopics} digits={0} />
          <AbbreviatedSingleStat title="Messages" value={s.numMessages} digits={0} />
          <AbbreviatedSingleStat title="Subscriptions" value={s.numSubscriptions} digits={0} />
          <AbbreviatedSingleStat title="Pending" value={s.numPending} digits={0} />
          <PercentageSingleStat title="Processed" value={s.percentageProcessed} digits={0} />
          <SizeSingleStat title="Memory RSS" value={s.memoryResidentSetSize} digits={1} />
        </div>

        {/* Interval Stats */}
        <div class="level">
          <h1 class="level-left">Interval</h1>
          <AbbreviatedSingleStat title="Topics" value={s.numTopicsInterval} digits={0} />
          <AbbreviatedSingleStat title="Messages" value={s.numMessagesInterval} digits={0} />
          <AbbreviatedSingleStat title="Expired" value={s.numExpiredInterval} digits={0} />
          <AbbreviatedSingleStat
            title="Subscriptions"
            value={s.numSubscriptionsInterval}
            digits={0}
          />
          <AbbreviatedSingleStat title="Pulled" value={s.numPulledInterval} digits={0} />
          <AbbreviatedSingleStat title="Acked" value={s.numAckedInterval} digits={0} />
          <SizeSingleStat title="Memory RSS" value={s.memoryResidentSetSizeInterval} digits={1} />
        </div>
        {/* All Time Stats */}
        <div class="level">
          <h1 class="level-left">All Time</h1>
          <AbbreviatedSingleStat title="Topics" value={s.numTopicsAllTime} digits={0} />
          <AbbreviatedSingleStat title="Messages" value={s.numMessagesAllTime} digits={0} />
          <AbbreviatedSingleStat title="Expired" value={s.numExpiredAllTime} digits={0} />
          <AbbreviatedSingleStat
            title="Subscriptions"
            value={s.numSubscriptionsAllTime}
            digits={0}
          />
          <AbbreviatedSingleStat title="Pulled" value={s.numPulledAllTime} digits={0} />
          <AbbreviatedSingleStat title="Acked" value={s.numAckedAllTime} digits={0} />
        </div>
      </div>
    </section>
  );
}
